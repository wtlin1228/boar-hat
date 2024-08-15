import styled from "styled-components";

const StyledButton = styled.button`
  padding-inline: 1.125rem;
  border-radius: 0.25rem;
  border: 0.0625rem solid transparent;
  height: 2.25rem;
  cursor: pointer;
  font-weight: 500;
  background: var(--background-color, #1971c2);
  color: var(--color, #ffffff);
`;

export const Button = ({ isError, children }) => {
  return (
    <StyledButton
      style={{
        "--background-color": isError ? "#E03131" : "#1971C2",
        "--color": isError ? "#000000" : "#FFFFFF",
      }}
    >
      {children}
    </StyledButton>
  );
};
