const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')
const bodyParser = require('body-parser')

app.use(cors())
app.use(bodyParser.json()) // for parsing application/json
app.use(bodyParser.urlencoded({ extended: true })) // for parsing application/x-www-form-urlencoded

app.get('/', (req, res) => {
  res.json('tutorial yew fetch')
})

app.get('/other', (req, res) => {
    res.send("other")
})

app.get('/batman', (req, res) => {
    var user = {
        name: "bruce",
        // superhero: "batman"
    }

    res.send(user)
})

app.post('/attack', (req, res) => {

    console.log(req.body)

    res.json('response dari api attack')


})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})