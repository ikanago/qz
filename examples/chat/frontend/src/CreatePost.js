import React, { useState } from "react";

const CreatePost = () => {
    const [name, setName] = useState("");
    const [text, setText] = useState("");

    const handleNameInput = event => {
        setName(event.target.value);
    };

    const handleTextInput = event => {
        setText(event.target.value);
    }

    return (
        <>
        <input onChange={handleNameInput} value={name} type="text"/>
        <input onChange={handleTextInput} value={text} type="text"/>
        </>
    )
}

export default CreatePost;