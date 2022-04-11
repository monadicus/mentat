import get from 'lodash/get';
import React, { memo } from 'react';
import { useLanguage } from './hooks';
import { LANGUAGES } from './languages';

/**
 * Internationalization function
 * @param name translation lookup path
 * @returns Translation for this path
 */
export const i18n = (
  name: string,
  language = localStorage.oocLanguage
): string =>
  get(LANGUAGES[language || 'en_US'], name) ?? get(LANGUAGES.en_US, name);

const I18nInner: React.FC<{ name: string }> = ({ name }) => (
  <>{i18n(name, useLanguage())}</>
);

/** Internationalization component */
export const I18n = memo(I18nInner);
