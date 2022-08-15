const comments = {
    async load() {
        const response = await fetch('/api/comments');
        const data = await response.json();
        let commentList = document.getElementById('comment-list');
        commentList.innerHTML = '';
        const template = document.getElementById('comment-template');
        if (data.length > 0) {
            data.forEach(comment => {
                let clone = template.content.cloneNode(true);
                clone.querySelector('.comment-author-name').textContent = comment.name;
                clone.querySelector('.comment-author-date').textContent = new Date(comment.date_posted).toLocaleString();
                clone.querySelector('.comment-body').textContent = comment.body;
                commentList.appendChild(clone);
            });
        } else {
            commentList.innerHTML = '<p>No comments yet</p>';
        }
    }
};