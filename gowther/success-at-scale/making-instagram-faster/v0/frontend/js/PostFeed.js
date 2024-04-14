document.querySelector("#PostFeed-js span").innerHTML = "✅";

const renderPosts = (posts) => {
  document.querySelector("main").append(...posts.map(generatePost));
};

const generatePost = (post) => {
  let article = document.createElement("article");
  article.innerHTML = `
            <img src="${post.image}" alt="${post.title}" />
            <h2>${post.title}</h2>
            <p>${post.body}</p>
          `;
  return article;
};

fetch("/api/posts").then((response) => {
  response.json().then((posts) => {
    document.querySelector("#get-posts-api span").innerHTML = "✅";
    renderPosts(posts);
  });
});
