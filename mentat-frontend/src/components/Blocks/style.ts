import styled from 'styled-components';

export const BlockIdStyle = styled.div`
  display: grid;
  grid-template-columns: minmax(max-content, 200px) auto;
  grid-auto-rows: auto;
  span {
    text-overflow: ellipsis;
  }
  label {
    grid-area: 1 / 1 / auto / span 2;
  }
`;
