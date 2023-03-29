import clsx from "clsx";
import { FC } from "react";

export type NavElemProps = {
  selected: boolean;
};

export const NavElem: FC<NavElemProps> = ({ selected, children }) => (
  <div
    className={clsx(
      "text-sm text-primary cursor-pointer",
      "opacity-50 hover:opacity-100 transition-opacity 100ms ease-in-out whitespace-nowrap",
      selected && "opacity-100"
    )}
  >
    {children}
  </div>
);
