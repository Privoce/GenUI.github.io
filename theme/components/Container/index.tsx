import { memo } from "react";
import styles from "./index.module.scss";

export interface ContainerProps {
  children: React.ReactNode;
}

const Container = memo(({ children }: ContainerProps) => {
  return <div className={styles.container} >{children}</div>;
});

export default Container;
