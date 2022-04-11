export const LANGUAGES = {
  en_US: require('../../../translations/en_US.yaml'),
};

console.debug('[debug] language', LANGUAGES);

export type Language = keyof typeof LANGUAGES;
