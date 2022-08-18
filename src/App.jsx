import React from "react";
import Comment from "./components/Comment.jsx";
import Toast from "./components/Toast.jsx";
import "./App.css";
import "./reset.css";

const USER = "Voronwë";
let evtSource = new EventSource("/api/upvotes");

export default function App() {
  let [body, setBody] = React.useState("");
  let [comments, setComments] = React.useState([]);
  let [showToast, setShowToast] = React.useState(false);

  const updateComment = (comment) => {
    setComments((comments) => {
      return comments.map((c) => {
        if (c.id == comment.id) {
          c.upvotes = comment.upvotes;
        }
        return c;
      });
    });
  };

  evtSource.onmessage = (e) => {
    updateComment(JSON.parse(e.data));
  };

  const fetchComments = async () => {
    const response = await fetch("/api/comments");
    const data = await response.json();
    setComments(data);
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
    const response = await fetch(`/api/comments/upvote/${id}`, {
      method: "POST"
    });
    const comment = await response.json();
    updateComment(comment);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    const data = {
      name: USER,
      body,
      parent_id: null
    };
    const response = await fetch("/api/comments", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });
    const comment = await response.json();
    setComments([...comments, comment]);
    toast();
  };

  return (
    <>
      <main className="comments">
        <h3 className="comments-title">Discussion</h3>
        <form className="comments-form" onSubmit={handleSubmit}>
          <div>
            <img className="avatar" src={`/images/${USER}.jpg`} alt={USER} />
          </div>
          <textarea
            className="comments-textarea"
            name="comment"
            id="comment"
            placeholder="What makest thou of this matter?"
            rows="1"
            required
            value={body}
            onChange={(e) => setBody(e.target.value)}
          ></textarea>
          <input type="submit" className="comments-submit" value="Comment" />
        </form>
        <div className="comments-list">
          {comments.length > 0 ? (
            comments.sort((a, b) => {
              if (a.parent_id && b.parent_id) {
                return a.parent_id - b.parent_id;
              } else if (a.parent_id == null) {
                return a.id - b.parent_id;
              } else if (b.parent_id == null) {
                return a.parent_id - b.id;
              }
              return b.upvotes - a.upvotes;
            }).map((comment) => (
              <Comment onUpvote={handleUpvote} key={comment.id} {...comment} />
            ))
          ) : (
            <p>No comments yet</p>
          )}
        </div>
      </main>
      <Toast show={showToast} />
    </>
  );
}
