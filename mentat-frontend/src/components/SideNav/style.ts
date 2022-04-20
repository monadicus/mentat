import styled from 'styled-components';
import media from 'styled-media-query';

interface SideNavProps {
  open: boolean;
}

export const SideNavStyle = styled.div<SideNavProps>`
  grid-area: sidenav;
  ${media.lessThan<SideNavProps>('medium')`
    position: fixed;
    background-color: ${props => props.theme.colors.dominant};
    width: 80%;
    height: 100%;
    z-index: 1;
    transform: translateX(${props => (props.open ? 0 : -100)}%);
    transition: transform 0.5s ease;
  `}

  .items {
    display: grid;
    grid-template-columns: auto 1fr;
    grid-auto-rows: ${props => props.theme.margin * 4}px;
    grid-gap: ${props => props.theme.margin}px 0;
    padding: ${props => props.theme.margin}px;
    align-items: stretch;

    a.nav-link {
      display: contents;

      &.active {
        .icon,
        .label {
          background-color: ${props => props.theme.colors.accent};
          color: ${props => props.theme.colors.main};
        }
      }

      .icon,
      .label {
        padding: 0 ${props => props.theme.margin}px;
        display: flex;
        align-items: center;
        color: ${props => props.theme.colors.accent};
      }
      .label {
        padding-left: 0;
      }
    }
  }
`;

export const SideNavShade = styled.div<SideNavProps>`
  pointer-events: ${props => (props.open ? 'initial' : 'none')};
  position: fixed;
  width: 100%;
  height: 100%;
  z-index: 1;

  background-color: ${props => props.theme.colors.secondary};
  opacity: ${props => (props.open ? 0.4 : 0)};
  transition: opacity 0.5s ease;
  cursor: pointer;
`;
