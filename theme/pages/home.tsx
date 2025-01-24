import { useI18n, useI18nUrl } from "../i18n";
import { normalizeImagePath, useNavigate } from "rspress/runtime";
import Hero from "../components/Home/Hero";
import { useCallback } from "react";

export function HomeLayout() {
  const t = useI18n();
  const tUrl = useI18nUrl();

  const navigate = useNavigate();
  const handleClickGetStarted = useCallback(() => {
    navigate(tUrl("/doc/guide/overview"));
  }, [tUrl, navigate]);

  const handleClickedMore = useCallback(() => {
    window.open("https://github.com/Privoce/GenUI", "_blank");
  }, []);

  return (
    <>
      <Hero
        logo={normalizeImagePath(t("logo"))}
        header={t("header")}
        logo_rs={normalizeImagePath(t("rs"))}
        title={t("heroTitle")}
        subTitle={t("heroSlogan")}
        description={t("heroSubSlogan")}
        getStartedButtonText={t("getStarted")}
        learnMoreButtonText={t("learnMore")}
        onClickGetStarted={handleClickGetStarted}
        onClickLearnMore={handleClickedMore}
      />
    </>
  );
}
