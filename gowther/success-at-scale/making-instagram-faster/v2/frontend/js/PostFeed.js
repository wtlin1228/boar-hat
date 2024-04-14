document.querySelector("#PostFeed-js span").innerHTML = "✅";

class PostFeed {
  constructor() {
    this.posts = [];
    this.currentIndex = 0;
  }

  setPosts = (posts) => {
    this.posts.push(...posts);
  };

  appendOnePost = () => {
    if (this.currentIndex >= this.posts.length) {
      // no post to append
      return;
    }

    document
      .querySelector("main")
      .append(PostFeed.generatePostElement(this.posts[this.currentIndex]));

    this.currentIndex += 1;
  };

  static generatePostImageSrcset = (image) => {
    return [
      `${image}-250w.webp 250w`,
      `${image}-500w.webp 500w`,
      `${image}.webp 800w`,
    ].join(", ");
  };

  static generatePostElement = (post) => {
    // We know the image is going to be in WebP format
    let imgSrcset = PostFeed.generatePostImageSrcset(
      post.image.slice(0, post.image.lastIndexOf(".webp"))
    );

    let article = document.createElement("article");
    article.innerHTML = `
      <img srcset="${imgSrcset}" src="${post.image}" alt="${post.title}" />
      <h2>${post.title}</h2>
      <p>${post.body}</p>
    `;

    return article;
  };
}

const postFeed = new PostFeed();

fetch("/api/posts").then((response) => {
  response.json().then((posts) => {
    document.querySelector("#get-posts-api span").innerHTML = "✅";
    postFeed.setPosts(posts);
    postFeed.appendOnePost();
  });
});
