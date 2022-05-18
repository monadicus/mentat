import styled from 'styled-components';

export const ErrorContainerStyle = styled.div<{ count: number }>`
  position: fixed;
  right: ${props => props.theme.margin}px;
  bottom: ${props => props.theme.margin}px;
  z-index: 1;
  display: flex;
  flex-direction: column;
  gap: ${props => props.theme.margin}px;

  .error-item {
    border: 1px solid ${props => props.theme.colors.error};
    overflow: hidden;
    max-height: 200px;

    .header {
      display: flex;
      align-items: center;
      background-color: ${props => props.theme.colors.error};
      padding-left: 8px;

      .code {
        margin: 0 8px;
      }

      .message {
        flex: 1;
      }
    }

    .content {
      padding: ${props => props.theme.margin}px;
      pre {
        margin: 0;
      }
    }

    &.hide {
      overflow: hidden;

      @keyframes hide {
        0% {
          transform: translate(0, 0);
          opacity: 1;
          max-height: 200px;
          margin-top: 0;
        }
        50% {
          transform: translate(50%, 0);
          opacity: 0;
          margin-top: 0;
          max-height: 200px;
        }
        100% {
          transform: translate(50%, 0);
          margin-top: -${props => props.theme.margin}px;
          max-height: 0;
        }
      }

      margin-top: -${props => props.theme.margin}px;
      opacity: 0;
      max-height: 0;

      animation: hide 0.5s ease 1;
    }
  }
`;

export const ErrorDismissButtonStyle = styled.div`
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: ${props => props.theme.margin * 2}px;
  height: ${props => props.theme.margin * 2}px;
`;
