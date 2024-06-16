use anyhow::Context;
use std::process::Stdio;
use tempfile::tempdir;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let docker_image = &args[2];
    let command = &args[3];
    let command_args = &args[4..];

    let (image_name, image_tag) = docker_image
        .split_once(':')
        .unwrap_or((docker_image, "latest"));

    let client = reqwest::blocking::Client::new();

    // requesting a token
    let token_requesting_url = format!(
        "https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/{}:pull",
        image_name
    );
    let v = client
        .get(&token_requesting_url)
        .send()
        .context(format!("GET {}", token_requesting_url))?
        .json::<serde_json::Value>()
        .context("deserialize response to json")?;
    let token = v["token"].as_str().context("get token")?;

    // fetch the image manifests
    let image_manifest_url = format!(
        "https://registry.hub.docker.com/v2/library/{}/manifests/{}",
        image_name, image_tag
    );
    let v = client
        .get(&image_manifest_url)
        .header(
            "Accept",
            "application/vnd.docker.distribution.manifest.v2+json",
        )
        .bearer_auth(token)
        .send()
        .context(format!("GET {}", image_manifest_url))?
        .json::<serde_json::Value>()
        .context("deserialize response to json")?;
    let layers = v["layers"].as_array().context("get layers")?;

    let temp_dir = tempdir().context("create tempdir")?;

    // pull layers
    for layer in layers {
        let digest = layer["digest"].as_str().context("get digest")?;
        let image_layer_url = format!(
            "https://registry.hub.docker.com/v2/library/{}/blobs/{}",
            image_name, digest
        );
        let blob = client
            .get(&image_layer_url)
            .bearer_auth(token)
            .send()
            .context(format!("GET {}", image_layer_url))?
            .bytes()
            .context("get bytes")?;
        tar::Archive::new(flate2::read::GzDecoder::new(&blob[..]))
            .unpack(temp_dir.path())
            .context(format!("unpack layer {}", digest))?;
    }

    // create /dev/null file
    let path_dev_null = temp_dir.path().join("dev/null");
    std::fs::create_dir_all(path_dev_null.parent().unwrap()).context("create dir /dev")?;
    std::fs::File::create(path_dev_null).context("create file /dev/null")?;

    // copy command
    let path_command = temp_dir
        .path()
        .join(command.strip_prefix("/").unwrap_or(command));
    std::fs::create_dir_all(path_command.parent().unwrap())
        .context(format!("create dir {:?}", path_command.parent().unwrap()))?;
    std::fs::copy(command, path_command).context("copy command")?;

    // chroot jail
    std::os::unix::fs::chroot(temp_dir.path()).context("chroot into temporary directory")?;

    // process isolation
    #[cfg(target_os = "linux")]
    unsafe {
        libc::unshare(libc::CLONE_NEWPID)
    };

    let status = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    }

    Ok(())
}
