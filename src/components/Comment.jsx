import React from "react";
import "./Comment.css";

export default function Comment(props) {
  const handleUpvote = async () => {
    const response = await fetch(`/api/comments/upvote/${props.id}`, {
        method: "POST",
      });
    const comment = await response.json();
    props.onUpvote(comment);
  };

  return (
    <div className="comment">
      <div className="comment-avatar">
        <img className="avatar" src={`/images/${props.name}.jpg`} />
      </div>
      <div className="comment-content">
        <div className="comment-header">
          <div className="comment-author">
            <span className="comment-author-name">{props.name}</span>⸱
            <span className="comment-author-date">{props.date_posted}</span>
          </div>
        </div>
        <div className="comment-body">
          <p>{props.body}</p>
        </div>
        <div className="comment-actions">
          <button className="comment-action-upvote" onClick={handleUpvote}>
            ⏶ Upvote (<span className="comment-upvote-count">{props.upvotes}</span>)
          </button>
          <button className="comment-action-reply" onclick="comments.reply(this)">
            Reply
          </button>
        </div>
      </div>
    </div>
  );
}
