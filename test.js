var request = require("request");
var options = {
  method: "GET",
  //url: "https://forio.com/lppi/prod/get_data",
  //url: "http://0.0.0.0:8080/",
  url: "https://multiple-bonny-applieddatacompany.koyeb.app/",
  time: true,
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    select: ["citizenship"],
    where: { race_recoded: 1 },
    groupBy: ["sex", "veteran", "latino_race"],
  }),
};

for (let i = 0; i < 10; i++) {
  request(options, function (error, response) {
    if (error) throw new Error(error);
    console.log(response.elapsedTime, response.body);
  });
}
