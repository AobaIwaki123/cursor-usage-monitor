import { useState } from 'react'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <h1>Cursor Usage Dashboard</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          Hello, count is {count}
        </button>
        <p>
          Frontend is running! Backend health check coming soon.
        </p>
      </div>
    </div>
  )
}

export default App