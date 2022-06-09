import { useSelector } from 'react-redux';
import { Language } from './languages';
import { selectLanguage } from './selectors';

export const useLanguage: () => Language = () => useSelector(selectLanguage);
