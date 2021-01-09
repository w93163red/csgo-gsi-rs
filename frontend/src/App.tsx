import React from 'react';
import styled from 'styled-components'
import { getPlayer } from './command/getdata';

function App() {
  const App = styled.div`
    min-height: 100vh;
  `;

  getPlayer();

  return (
    <App>
      hello world
    </App>
  );
}

export default App;
