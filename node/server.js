const express = require('express');
const { markov } = require('../pkg/markov_lib.js');

const app = express();
const port = 3000;
app.use(express.static('./node/public'));
app.use(express.urlencoded({ extended: false }));

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/markov', function (req, res) {
  console.log("Received \"" + req.body["input_text"] + "\"");
  console.log(req.body["count"]);

  res.send(markov(req.body["input_text"], parseInt(req.body["count"])))
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))
