import React, { useState } from "react";

const apiBaseUrl = "http://localhost:8080";

const CreatePost = () => {
    const [name, setName] = useState("");
    const [text, setText] = useState("");

    const handleNameInput = event => {
        setName(event.target.value);
    };

    const handleTextInput = event => {
        setText(event.target.value);
    }

    const clearInput = () => {
        setName("");
        setText("");
    }

    const makeRequest = () => {
        const xhr = new XMLHttpRequest();
        xhr.responseType = "json";
        xhr.open("POST", apiBaseUrl + "/create_post");
        xhr.send(JSON.stringify({
            username: name,
            text: text,
        }));

        xhr.onload = _e => {
            if (xhr.status === 200) {
                clearInput();
            }
        };
    }

    return (
        <>
        <input onChange={handleNameInput} value={name} type="text"/>
        <input onChange={handleTextInput} value={text} type="text"/>
        <button onClick={makeRequest} >Send</button>
        </>
    )
}

export default CreatePost;