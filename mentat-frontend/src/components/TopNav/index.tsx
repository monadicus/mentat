import React from 'react';
import { FaGithub } from 'react-icons/fa';
import { useSelector } from 'react-redux';
import styled from 'styled-components';
import { i18n } from '../../features/i18n/components';
import { selectNetworkVersions } from '../../features/rosetta/selectors';
import { MenuButton } from './MenuButton';

export const TopNavStyle = styled.div`
  background-color: ${props => props.theme.colors.main};
  color: ${props => props.theme.colors.accent};
  height: ${props => props.theme.margin * 4}px;
  display: flex;
  align-items: center;
  padding: 0 ${props => props.theme.margin}px;
  grid-area: topnav;

  .logo {
    font-size: 24px;
    line-height: 1;
  }

  .flex {
    flex-grow: 1;
  }

  .links {
  }

  .versions {
    font-family: monospace;
    margin-left: ${props => props.theme.margin}px;
    opacity: 0.5;
    font-size: 8px;
  }

  a,
  a:visited,
  a:active {
    color: ${props => props.theme.colors.accent};
  }
`;

export const TopNav: React.FC = () => {
  const version = useSelector(selectNetworkVersions);

  return (
    <TopNavStyle>
      <MenuButton />
      <div className="logo">mentat</div>
      {version && (
        <div className="versions">
          {Object.entries(version).map(([key, version]) => (
            <div key={key}>
              <>
                {key.replace(/_version$/, '')} = v{version}
              </>
            </div>
          ))}
        </div>
      )}
      <div className="flex" />
      <div className="links">
        <a
          href="https://github.com/monadicus/mentat"
          target="_blank"
          rel="noopener noreferrer"
          aria-label={i18n('navigation.header.links_github_aria')}
          title={i18n('navigation.header.links_github_title')}
        >
          <FaGithub />
        </a>
      </div>
    </TopNavStyle>
  );
};
