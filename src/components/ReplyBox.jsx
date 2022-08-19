import React from "react";
import UserContext from "../UserContext";
import "./ReplyBox.css";

export default function ReplyBox(props) {
  const user = React.useContext(UserContext);
  return (
    <form className="comments-form" onSubmit={props.onSubmit}>
      <div>
        <img className="avatar" src={`/images/${user}.jpg`} alt={user} />
      </div>
      <textarea
        className="comments-textarea"
        placeholder={
          props.isReply ? "What is thy answer?" : "What makest thou of this matter?"
        }
        rows="1"
        required
        value={props.value}
        onChange={props.onChange}
      ></textarea>
      <input type="submit" className="comments-submit" value="Comment" />
    </form>
  );
}
