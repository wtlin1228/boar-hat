import React from "react";
import "./ProviderComponent.css";
import _ from "lodash";

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
      <h1 className="title">{_.join(["Pikmin", "Federation", "2.0"], " ")}</h1>
    </div>
  );
};

export default Provider;
