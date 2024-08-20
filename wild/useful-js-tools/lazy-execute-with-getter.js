function createReader() {
  let value = null;

  function getExpensiveValue() {
    if (value !== null) return value;

    value = "some expensive value";
    return value;
  }

  return {
    get somethingExpensive() {
      return getExpensiveValue();
    },
  };
}

const reader = createReader();

// `getExpensiveValue()` is only executed when this property gets read
console.log(reader.somethingExpensive);
