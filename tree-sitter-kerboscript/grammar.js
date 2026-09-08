module.exports = grammar({
  name: "kerboscript",
  rules: {
    source_file: $ => repeat($._token),
    _token: $ => /\S+/,
  },
});
