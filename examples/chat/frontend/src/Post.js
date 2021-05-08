import React from "react";

const Post = props => {
    return (
        <div>
            <p>{props.username}</p>
            <p>{props.text}</p>
        </div>
    )
}

export default Post;
