import { useLang, usePageData } from "rspress/runtime";
import Theme from "rspress/theme";
// import Announcement from './components/Announcement';
import { HomeLayout } from "./pages/home";
import { useI18nUrl } from "./i18n";

const Layout = () => {
  const { page } = usePageData();
  const lang = useLang();
  const tUrl = useI18nUrl();
  const ANNOUNCEMENT_URL = tUrl("/guide/start/introduction");
  return <Theme.Layout />;
};

export * from "rspress/theme";

export default {
  ...Theme,
  Layout,
  HomeLayout,
  useI18nUrl,
};
