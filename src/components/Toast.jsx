import React from "react";
import "./Toast.css";

export default function Toast(props) {
  React.useEffect(() => {}, [props.show]);

  return <div className={"toast " + (!props.show && "hidden")}>Comment added</div>;
}
