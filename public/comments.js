const comments = {
  renderComment(comment, node) {
    try {
      node.querySelector(".comment").dataset.comment = JSON.stringify(comment);
    } catch (e) {
      node.dataset.comment = JSON.stringify(comment);
    }
    node.querySelector(".avatar").src = "/images/" + comment.name + ".jpg";
    node.querySelector(".comment-author-name").textContent = comment.name;
    node.querySelector(".comment-author-date").textContent = new Date(
      comment.date_posted
    ).toLocaleString();
    node.querySelector(".comment-body").textContent = comment.body;
    node.querySelector(".comment-upvote-count").textContent =
      comment.upvotes.toString();
  },

  async load() {
    const response = await fetch("/api/comments");
    const data = await response.json();
    let commentList = document.getElementById("comment-list");
    commentList.innerHTML = "";
    const template = document.getElementById("comment-template");
    if (data.length > 0) {
      data.forEach((comment) => {
        let clone = template.content.cloneNode(true);
        this.renderComment(comment, clone);
        commentList.appendChild(clone);
      });
    } else {
      commentList.innerHTML = "<p>No comments yet</p>";
    }
  },

  async submit(form) {
    let commentTextarea = form.querySelector("#comment");
    const data = {
      name: form.querySelector("#name").value,
      comment: commentTextarea.value,
    };
    const response = await fetch("/api/comments", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    let commentList = document.getElementById("comment-list");
    const template = document.getElementById("comment-template");
    const clone = template.content.cloneNode(true);
    this.renderComment(await response.json(), clone);
    commentList.appendChild(clone);
    commentTextarea.value = "";
    commentTextarea.focus();
    document.querySelector("#toast").classList.remove("hidden");
    setTimeout(() => {
      document.querySelector("#toast").classList.add("hidden");
    }, 2500);
  },

  async upvote(node) {
    let commentNode = node.closest(".comment");
    const commentId = JSON.parse(commentNode.dataset.comment).id;
    const response = await fetch(`/api/comments/upvote/${commentId}`, {
      method: "POST",
    });
    this.renderComment(await response.json(), commentNode);
  },
};
