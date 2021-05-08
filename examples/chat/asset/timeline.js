const apiBaseUrl = "http://localhost:8080";

const createPost = () => {
    const username = document.getElementById("username").value;
    const text = document.getElementById("text").value;
    const data = {
        username: username,
        text: text,
    };

    const xhr = new XMLHttpRequest();
    xhr.responseType = "json";
    xhr.open("POST", apiBaseUrl + "/create_post");
    xhr.send(JSON.stringify(data));
}

const getPosts = () => {
    const xhr = new XMLHttpRequest();
    xhr.responseType = "json";
    xhr.onload = () => {
        if (xhr.status == 200) {
            const timeline = document.getElementById("timeline");
            const posts = xhr.response;
            const renderedPosts = posts["posts"].map(post => {
                const username = document.createElement("p");
                username.append(post.username);
                const text = document.createElement("p");
                text.append(post.text);
                const postNode = document.createElement("div");
                postNode.append(username, text);
                return postNode;
            });
            timeline.append(...renderedPosts);
        }
    };

    xhr.open("GET", apiBaseUrl + "/posts");
    xhr.send(null);
}

window.onload = getPosts;

const createPostButton = document.getElementById("__createPost");
createPostButton.addEventListener("click", createPost);
