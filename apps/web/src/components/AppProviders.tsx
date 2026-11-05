import React, { type PropsWithChildren } from "react";
import { store } from "../store/store";
import { Provider } from "jotai";

type Props = {};

const AppProviders = ({ children, ...props }: PropsWithChildren<Props>) => {
  return (
    <Provider store={store} {...props}>
      {children}
    </Provider>
  );
};

export default AppProviders;
