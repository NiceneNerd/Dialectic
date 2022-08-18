import React from "react";
import Comment from "./components/Comment.jsx";
import "./App.css";
import "./reset.css";
import Toast from "./components/Toast.jsx";

const USER = "Voronwë";

export default function App() {
  let [body, setBody] = React.useState("");
  let [comments, setComments] = React.useState([]);
  let [showToast, setShowToast] = React.useState(false);

  let evtSource = new EventSource("/api/upvotes");
  const updateComment = (comment) => {
    console.log(comments);
    setComments((comments) => {
      return comments.map((c) => {
        if (c.id == comment.id) {
          c.upvotes = comment.upvotes;
        }
        return c;
      });
    });
    if (evtSource.readyState == evtSource.CLOSED) {
      console.log("Reloading event stream", comments);
      evtSource = new EventSource("/api/upvotes");
      evtSource.onmessage = onUpvoteMsg;
    }
  };
  const onUpvoteMsg = (e) => {
    console.log(e, comments);
    const upvoteData = JSON.parse(e.data);
    updateComment(upvoteData);
  };
  evtSource.onmessage = onUpvoteMsg;

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
    evtSource.close();
    evtSource.onmessage = null;
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
            comments.map((comment) => (
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
