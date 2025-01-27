import { memo } from "react";
import styles from "./index.module.scss";

export interface ColorProps {
  title: string;
  colors: string[];
}

const Color = memo(({ title, colors }: ColorProps) => {
  return <div className={styles.color} style={{ backgroundColor: colors[6] }}>
    <div className={styles["color-info"]}>
      <h2>{title}</h2>
      <h3>{colors[6]}</h3>
    </div>
    <div className={styles["color-blocks"]}>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[0] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[1] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[2] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[3] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[4] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[5] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[6] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[7] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[8] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[9] }}></div>
      <div className={styles["color-blocks-item"]} style={{ backgroundColor: colors[10] }}></div>
    </div>
  </div>;
});

export default Color;
