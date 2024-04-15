document.querySelector("#PostFeed-js span").innerHTML = "✅";

const POST_LOADING_STATUS = {
  NOT_LOADED: "not-loaded",
  LOADED: "loaded",
  LAZY_LOADING: "lazy-loading",
  EAGER_LOADING: "eager-loading",
};

class PostFeed {
  constructor() {
    this.posts = [];
    // Array<
    //   | [POST_LOADING_STATUS.NOT_LOADED]
    //   | [POST_LOADING_STATUS.LOADED]
    //   | [POST_LOADING_STATUS.LAZY_LOADING, requestIdleCallbackId]
    //   | [POST_LOADING_STATUS.EAGER_LOADING]
    // >
    this.postLoadingStatus = [];
    this.loadedPostCount = 0;
    this.loadedAndLoadingPostCount = 0;
  }

  setPosts = (posts) => {
    this.posts.push(...posts);
    this.postLoadingStatus = Array.from({ length: posts.length }, () => [
      POST_LOADING_STATUS.NOT_LOADED,
    ]);
  };

  setupLoadMoreOnScroll = () => {
    let options = {
      root: document,
      rootMargin: "0px",
      threshold: 1.0,
    };

    let callback = (entries, observer) => {
      if (this.loadedPostCount >= this.posts.length) {
        // no post to append
        observer.unobserve(document.querySelector("footer"));
        return;
      }

      // let's load one post eagerly and two posts lazily
      this.loadPostEagerly();
      this.loadPostLazily();
      this.loadPostLazily();
    };

    let observer = new IntersectionObserver(callback, options);

    observer.observe(document.querySelector("footer"));
  };

  loadPostLazily = () => {
    const post_id_to_load = this.loadedAndLoadingPostCount;
    if (post_id_to_load >= this.posts.length) {
      return;
    }

    console.log("load post lazily", post_id_to_load);
    const [status] = this.postLoadingStatus[post_id_to_load];

    if (status !== POST_LOADING_STATUS.NOT_LOADED) {
      return;
    }

    this.postLoadingStatus[post_id_to_load] = [
      POST_LOADING_STATUS.LAZY_LOADING,
      requestIdleCallback(() => {
        document
          .querySelector("main")
          .append(this.generatePostElement(post_id_to_load));
        this.loadedPostCount += 1;
      }),
    ];
    this.loadedAndLoadingPostCount += 1;
  };

  loadPostEagerly = () => {
    let post_id_to_load = this.loadedPostCount;
    if (post_id_to_load >= this.posts.length) {
      return;
    }

    console.log("load post eagerly", post_id_to_load);
    const [status, id] = this.postLoadingStatus[post_id_to_load];

    if (status === POST_LOADING_STATUS.LOADED) {
      return;
    }

    if (status === POST_LOADING_STATUS.LAZY_LOADING) {
      cancelIdleCallback(id);
    } else {
      this.loadedAndLoadingPostCount += 1;
    }

    this.postLoadingStatus[post_id_to_load] = [
      POST_LOADING_STATUS.EAGER_LOADING,
    ];
    document
      .querySelector("main")
      .append(this.generatePostElement(post_id_to_load));
    this.loadedPostCount += 1;
  };

  static generatePostImageSrcset = (image) => {
    return [
      `${image}-250w.webp 250w`,
      `${image}-500w.webp 500w`,
      `${image}.webp 800w`,
    ].join(", ");
  };

  generatePostElement = (postIdx) => {
    const post = this.posts[postIdx];
    if (!post) {
      return;
    }

    // We know the image is going to be in WebP format
    let imgSrcset = PostFeed.generatePostImageSrcset(
      post.image.slice(0, post.image.lastIndexOf(".webp"))
    );

    let article = document.createElement("article");
    article.style.display = "none";

    let img = document.createElement("img");
    img.srcset = imgSrcset;
    img.src = post.image;
    img.alt = post.title;
    img.style.aspectRatio = post.imageAspectRatio;
    img.style.width = "100%";
    img.onload = () => {
      // don't bother handling the order in this demo project
      article.style.display = "block";
      this.postLoadingStatus[postIdx] = [POST_LOADING_STATUS.LOADED];
    };
    article.append(img);

    const h2 = document.createElement("h2");
    h2.innerText = post.title;
    article.append(h2);

    const p = document.createElement("p");
    p.innerText = post.body;
    article.append(p);

    return article;
  };
}

const postFeed = new PostFeed();

fetch("/api/posts").then((response) => {
  response.json().then((posts) => {
    document.querySelector("#get-posts-api span").innerHTML = "✅";
    postFeed.setPosts(posts);
    postFeed.setupLoadMoreOnScroll();
  });
});
