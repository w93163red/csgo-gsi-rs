import React from "react";
import styled from "styled-components";

interface ScoreProps {
    ct: number;
    t: number;
}

export const Score = (props: ScoreProps) => {
    const ScoreBoard = styled.div`
        font-size: 60px;
        color: black;
        position: relative;
        margin: 0 auto;
    `;

    const CT = styled.span`
        font-size: 60px;
        color: #83C0EF;
        opacity: 0.8;
    `;

    const T = styled.span`
        font-size: 60px;
        color: #DFCA6E;
        opacity: 0.8;
    `;

    return (<ScoreBoard>
        <CT>COUNTER-TERROIRSTS {props.ct}</CT>
        <T>{props.t} TERRORISTS</T>

    </ScoreBoard>);
}