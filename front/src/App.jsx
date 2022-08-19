import React from "react";
import Comment from "./components/Comment.jsx";
import ReplyBox from "./components/ReplyBox.jsx";
import Toast from "./components/Toast.jsx";
import { UserProvider } from "./UserContext.js";
import "./App.css";
import "./reset.css";

const USER = "Voronwë";
let evtSource = new EventSource("/api/upvotes");

export default function App() {
  let [body, setBody] = React.useState("");
  let [comments, setComments] = React.useState([]);
  let [showToast, setShowToast] = React.useState(false);
  let [showError, setShowError] = React.useState(false);
  let [errorMessage, setErrorMessage] = React.useState("");

  const nestedUpdate = (comment, id, upvotes) => {
    if (comment.id == id) {
      comment.upvotes = upvotes;
    } else {
      comment.children = comment.children.map((child) =>
        nestedUpdate(child, id, upvotes)
      );
    }
    return comment;
  };

  const updateComment = (comment) => {
    setComments((comments) => {
      return comments.map((c) => {
        return nestedUpdate(c, comment.id, comment.upvotes);
      });
    });
  };

  evtSource.onmessage = (e) => {
    updateComment(JSON.parse(e.data));
  };

  const fetchComments = async () => {
    const response = await fetch("/api/comments");
    if (response.ok) {
      setComments(await response.json());
    } else {
      setShowError(true);
      setErrorMessage(await response.text());
    }
  };

  React.useEffect(() => {
    fetchComments();
  }, []);

  const toast = () => {
    setShowToast(true);
    setTimeout(() => {
      setShowToast(false);
    }, 2500);
  };

  const handleUpvote = async (id) => {
    try {
      const response = await fetch(`/api/comments/upvote/${id}`, {
        method: "POST",
      });
      if (!response.ok) {
        throw new Error(await response.text());
      }
      const comment = await response.json();
      updateComment(comment);
    } catch (error) {
      setShowError(true);
      setErrorMessage(error.message);
    }
  };

  const handleSubmit = async (e, parent_id, commentBody) => {
    e.preventDefault();
    const data = {
      name: USER,
      body: commentBody || body,
      parent_id,
    };
    try {
      const response = await fetch("/api/comments", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      if (!response.ok) {
        throw new Error(await response.text());
      }
    } catch (error) {
      setShowError(true);
      setErrorMessage(await response.text());
      return;
    }
    fetchComments();
    setBody("");
    toast();
  };

  return (
    <>
      <main className="comments">
        <h3 className="comments-title">Discussion</h3>
        {!showError ? (
          <UserProvider value={USER}>
            <ReplyBox
              onSubmit={handleSubmit}
              onChange={(e) => setBody(e.target.value)}
              value={body}
            />
            <div className="comments-list">
              {comments.length > 0 ? (
                comments.map((comment) => (
                  <Comment
                    onUpvote={handleUpvote}
                    onReply={handleSubmit}
                    key={comment.id}
                    {...comment}
                  />
                ))
              ) : (
                <p>No comments yet</p>
              )}
            </div>
          </UserProvider>
        ) : (
          <div className="error">
            <h5>Error</h5>
            <p>{errorMessage}</p>
            <p><a href="/">Reload</a></p>
          </div>
        )}
      </main>
      <Toast show={showToast} />
    </>
  );
}
