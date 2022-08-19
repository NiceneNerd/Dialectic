import React from "react";
import ReplyBox from "./ReplyBox.jsx";
import prettyDate from "../pretty.js";
import "./Comment.css";

export default function Comment(props) {
  let [showReply, setShowReply] = React.useState(false);
  let [replyBody, setReplyBody] = React.useState("");

  const handleReply = (e) => {
    e.preventDefault();
    props.onReply(e, props.id, replyBody);
    setShowReply(false);
  };

  return (
    <>
      <div className={"comment " + (props.parentId && "nested")}>
        <div className="comment-avatar">
          <img className="avatar" src={`/images/${props.name}.jpg`} />
        </div>
        <div className="comment-content">
          <div className="comment-header">
            <div className="comment-author">
              <span className="comment-author-name">{props.name}</span> ⸱{" "}
              <span className="comment-author-date">{prettyDate(props.date_posted)}</span>
            </div>
          </div>
          <div className="comment-body">
            <p>{props.body}</p>
          </div>
          <div className="comment-actions">
            <button
              className="comment-action-upvote"
              onClick={() => props.onUpvote(props.id)}
            >
              ⏶ Upvote (<span className="comment-upvote-count">{props.upvotes}</span>)
            </button>
            <button
              className="comment-action-reply"
              onClick={() => setShowReply(!showReply)}
            >
              Reply
            </button>
          </div>
          {showReply && (
            <ReplyBox
              onSubmit={handleReply}
              onChange={(e) => setReplyBody(e.target.value)}
              isReply={true}
              value={replyBody}
              user={props.user}
            />
          )}
        </div>
      </div>
      {props.children.map((child) => (
        <Comment
          onReply={props.onReply}
          onUpvote={props.onUpvote}
          parentId={props.id}
          {...child}
          key={child.id}
        />
      ))}
    </>
  );
}
