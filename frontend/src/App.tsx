import React, { useEffect, useState } from 'react';
import styled from 'styled-components'
import { getState } from './command/getdata';

function App() {
  // state
  const [gameState, setgameState] = useState({});

  const App = styled.div`
    min-height: 100vh;
  `;

  useEffect(() => {
    async function getGameState() {
      const gs = await getState();
      setgameState(gs);
    }
    getGameState();
  }, [])


  return (
    <App>
      {JSON.stringify(gameState)}
    </App>
  );
}

export default App;
