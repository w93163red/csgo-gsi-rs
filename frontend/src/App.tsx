import React, { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';
import styled from 'styled-components'
import { getState } from './command/getdata';
import { RootState } from './store/store';
import { Layout } from 'antd';
import { Score } from './score';
 // eslint-disable-next-line
const { Header, Footer, Content } = Layout;

function App() {
  // state
   // eslint-disable-next-line
  const [gameState, setgameState] = useState({});
  const currentGameState = useSelector((state: RootState) => state.gamestate);
  const App = styled.div`
    min-height: 100vh;
  `;

  const Header = styled.header`
    background: rgba(0,0,0,0.6);
    text-align: center;
    vertical-align: text-bottom;
  `;

  const Footer = styled.footer`
    color: #fff;
    background: #7dbcea;
    min-height: 100px;
    position: fixed;
    bottom: 0;
    width: 100%;
    `;

  useEffect(() => {
    setgameState(currentGameState);
    async function wrapper() {
      await getState();
    }
    wrapper();
  }, [currentGameState])
  return (
    <App>
      <Layout>
        <Header>
          <Score ct={1} t={1} />
        </Header>
        <Content>Content</Content>
        <Footer>fuck</Footer>
      </Layout>
    </App>
  );
}

export default App;
