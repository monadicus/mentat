import styled from 'styled-components';

export const BlockIdStyle = styled.div`
  display: grid;
  grid-template-columns: minmax(max-content, 100px) auto;
  grid-auto-rows: auto;
  margin: ${props => props.theme.margin}px;
  .block-hash {
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    max-width: 200px;
  }
  label {
    grid-area: 1 / 1 / auto / span 2;
  }
`;
