import clsx from "clsx";
import { FC } from "react";

export type NavElemProps = {
  selected: boolean;
  onClick?: () => void;
};

export const NavElem: FC<NavElemProps> = ({ selected, children, onClick}) => (
  <div
    className={clsx(
      "text-sm text-primary cursor-pointer",
      "opacity-50 hover:opacity-100 transition-opacity 100ms ease-in-out whitespace-nowrap",
      selected && "opacity-100"
    )}
    onClick={onClick}
  >
    {children}
  </div>
);
