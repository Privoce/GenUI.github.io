import { memo, useCallback } from "react";
import styles from "./index.module.scss";
import { useNavigate } from "rspress/runtime";
import { useI18nUrl } from "../../i18n";

export interface LinkCardProps {
  href: string;
  title: string;
  description?: string;
}

const MyLinkCard = memo(({ href, title, description }: LinkCardProps) => {
  const tUrl = useI18nUrl();
  const navigate = useNavigate();
  const onClicked = useCallback((href: string): undefined => {
    console.log("onClicked");
    navigate(tUrl(href));
  }, [navigate, tUrl]);
  
  return (
    <div className={styles.card} onClick={() => onClicked(href)}>
      <div className={styles["card-icon"]}>
        <svg
          width="18"
          height="18"
          viewBox="0 0 48 48"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M24 44C35.0457 44 44 35.0457 44 24C44 12.9543 35.0457 4 24 4C12.9543 4 4 12.9543 4 24C4 35.0457 12.9543 44 24 44Z"
            fill="none"
            stroke="#FF7043"
            stroke-width="4"
            stroke-linejoin="round"
          />
          <path
            d="M21 33L30 24L21 15"
            stroke="#FF7043"
            stroke-width="4"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <h1 className={styles["card-title"]}>{title}</h1>
      <p className={styles["card-desc"]}>{description}</p>
    </div>
  );
});

export default MyLinkCard;