import React, { useEffect, useState } from "react";
import Post from "./Post";

const apiBaseUrl = "http://localhost:8080";

const Posts = () => {
    const [posts, setPosts] = useState([]);
    
    useEffect(() => {
        const xhr = new XMLHttpRequest();
        xhr.responseType = "json";
        xhr.onload = _e => {
            if (xhr.status === 200) {
                const response = xhr.response;
                setPosts(response["posts"]);
            }
        };
        xhr.open("GET", apiBaseUrl + "/posts");
        xhr.send(null);
    }, []);

    return (
    <>
        {posts.map(post => <Post username={post.username} text={post.text}/>)}
    </>
    );
}

export default Posts;