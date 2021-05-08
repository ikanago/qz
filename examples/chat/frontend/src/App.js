import './App.css';
import CreatePost from "./CreatePost";
import Posts from './Posts';

function App() {
  return (
    <div className="App">
      <header className="App-container">
        <Posts/>
        <CreatePost/>
      </header>
    </div>
  );
}

export default App;
