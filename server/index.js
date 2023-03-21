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


  let is_password_correct = true;

  if (is_password_correct) {
    res.status(200).json("attack berhasil")
  } else {
    res.status(400).json("gagal")
  }

})


app.get('/schedules/:id', (req, res) => {
  console.log(req.params);

  var schedules;

  var id = req.params.id;

  if (id == '"1"') {
    schedules = [
      {
        task: "save gotham",
        superhero: "batman",
        is_on_going: true,
      },
      {
        task: "fly",
        superhero: "superman",
        is_on_going: true,
      },
      {
        task: "live 100 years",
        superhero: "wonder woman",
        is_on_going: true,
      },
      {
        task: "run",
        superhero: "flash",
        is_on_going: true,
      },
      {
        task: "find atlantis city",
        superhero: "aquaman",
        is_on_going: false,
      }
    ];
  } else {
    schedules = [
      {
        task: "save universe",
        superhero: "ironman",
        is_on_going: true,
      },
      {
        task: "fly",
        superhero: "thor",
        is_on_going: true,
      },
    ];
  }

  


  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    let data = {
      list: schedules,
      world: "DC"
    };
    res.status(200).send(data)
  }
})



app.get('/tasks/:id', (req, res) => {


  let token = "wrong token";




  let list = [
    {
      name: "bruce",
      status: "single",
    },
    {
      name: "clark",
      status: "single",
    }
  ]

  let data = {
    list: list,
    nationality: "hollywood",
  }






  if (token == "abc") {

    res.status(200).send(data)

  } else {

    let error = {
      error_description: "token is not verified"
    };


    res.status(400).send(error)
  }


})


app.post('/login-app', (req, res) => {
  console.log(req.body)


  // res.status(200).json("data from server")

  res.status(400).json("status 400")
})



app.get('login-app-get', (req, res) => {


  // CONNECT KE DATABASE
  // GET DATA DARI DATABASE
  // RETURN DATA KE FE
})






app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})