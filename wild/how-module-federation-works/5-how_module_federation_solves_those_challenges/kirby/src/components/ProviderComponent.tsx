import React from "react";
import _ from "lodash";
// this import CSS will cause error when host loads this remote
import "./ProviderComponent.css";

const Provider: React.FC = () => {
  return (
    <div className="container">
      <div className="icon-container">
        <img
          src="https://module-federation.io/svg.svg"
          alt="logo"
          className="logo-image"
        />
      </div>
      <h1 className="title">Kirby Federation {_.join([2, 0], ".")}</h1>
    </div>
  );
};

export default Provider;
