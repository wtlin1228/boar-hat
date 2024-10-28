import * as React from "react";

const styles: Record<string, React.CSSProperties> = {
  board: {
    height: "100vh",
    display: "flex",
    flexDirection: "row",
    padding: "20px",
    gap: "12px",
  },
  column: {
    minWidth: "350px",
    backgroundColor: "black",
    border: "1px solid #3d444d",
    borderRadius: "6px",
    display: "flex",
    flexDirection: "column",
    overflowY: "hidden",
  },
  title: {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    fontSize: "40px",
  },
  card: {
    border: "1px solid #3d444d",
    borderRadius: "6px",
    marginBottom: "8px",
    minHeight: "100px",
    fontSize: "24px",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
  },
  dropzone: {
    display: "flex",
    flexDirection: "column",
    overflowY: "auto",
    scrollPaddingBottom: "7px",
    flexGrow: 1,
    padding: "4px 8px",
  },
  draggable: {
    cursor: "grab",
  },
} as const;

type Card = {
  id: string;
  name: string;
  column: "todo" | "doing" | "done";
};

type State = {
  todo: string[];
  doing: string[];
  done: string[];
  cards: Record<string, Card>;
};

type Action = {
  type: "move_card";
  moveToColumn: "todo" | "doing" | "done";
  cardId: string;
};

function reducer(state: State, action: Action) {
  switch (action.type) {
    case "move_card": {
      const { cardId, moveToColumn } = action;

      const originalColumn = state.cards[cardId].column;
      if (originalColumn === moveToColumn) {
        return state;
      }

      // Create new arrays to maintain immutability
      const updatedOriginalColumn = state[originalColumn].filter(
        (id) => id !== cardId
      );
      const updatedTargetColumn = [...state[moveToColumn], cardId];

      // Update card column and return the new state
      return {
        ...state,
        [originalColumn]: updatedOriginalColumn,
        [moveToColumn]: updatedTargetColumn,
        cards: {
          ...state.cards,
          [cardId]: {
            ...state.cards[cardId],
            column: moveToColumn,
          },
        },
      };
    }
    default:
      throw Error("Unknown action: " + action.type);
  }
}

function App() {
  const [state, dispatch] = React.useReducer(reducer, {
    todo: ["default-card-0", "default-card-1", "default-card-2"],
    doing: ["default-card-3", "default-card-4"],
    done: ["default-card-5"],
    cards: {
      "default-card-0": { id: "default-card-0", name: "hawk", column: "todo" },
      "default-card-1": { id: "default-card-1", name: "kirby", column: "todo" },
      "default-card-2": { id: "default-card-2", name: "wild", column: "todo" },
      "default-card-3": {
        id: "default-card-3",
        name: "oatchi",
        column: "doing",
      },
      "default-card-4": {
        id: "default-card-4",
        name: "pikmin",
        column: "doing",
      },
      "default-card-5": {
        id: "default-card-5",
        name: "waddle dee",
        column: "done",
      },
    },
  });

  const moveCard = React.useCallback(
    (cardId: string, dest: "todo" | "doing" | "done") => {
      dispatch({
        type: "move_card",
        cardId,
        moveToColumn: dest,
      });
    },
    []
  );

  return (
    <div style={styles.board}>
      <div style={styles.column}>
        <h3 style={styles.title}>Todo</h3>
        <Dropzone moveCard={moveCard} column="todo">
          {state.todo.map((cardId) => (
            <Draggable id={cardId} key={cardId}>
              <div style={styles.card}>{state.cards[cardId].name}</div>
            </Draggable>
          ))}
        </Dropzone>
      </div>
      <div style={styles.column}>
        <h3 style={styles.title}>Doing</h3>
        <Dropzone moveCard={moveCard} column="doing">
          {state.doing.map((cardId) => (
            <Draggable id={cardId} key={cardId}>
              <div style={styles.card}>{state.cards[cardId].name}</div>
            </Draggable>
          ))}
        </Dropzone>
      </div>
      <div style={styles.column}>
        <h3 style={styles.title}>Done</h3>
        <Dropzone moveCard={moveCard} column="done">
          {state.done.map((cardId) => (
            <Draggable id={cardId} key={cardId}>
              <div style={styles.card}>{state.cards[cardId].name}</div>
            </Draggable>
          ))}
        </Dropzone>
      </div>
    </div>
  );
}

function Draggable({ id, children }: React.PropsWithChildren<{ id: string }>) {
  const dragstartHandler = React.useCallback(
    (ev: React.DragEvent<HTMLDivElement>) => {
      ev.dataTransfer.setData("text/plain", id);
    },
    [id]
  );

  return (
    <div
      id={id}
      draggable
      style={styles.draggable}
      onDragStart={dragstartHandler}
    >
      {children}
    </div>
  );
}

function Dropzone({
  moveCard,
  column,
  children,
}: React.PropsWithChildren<{
  moveCard: (cardId: string, dest: "todo" | "doing" | "done") => void;
  column: "todo" | "doing" | "done";
}>) {
  const handleDragOver = React.useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
    },
    []
  );

  const handleDrop = React.useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      const cardId = e.dataTransfer.getData("text");
      moveCard(cardId, column);
    },
    [moveCard, column]
  );

  return (
    <div
      style={styles.dropzone}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
    >
      {children}
    </div>
  );
}

export default App;
