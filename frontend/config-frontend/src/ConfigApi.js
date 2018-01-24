import request from 'superagent';

export default {
  triggerEvent: (eventType) => {
    var url = `http://localhost:8000/triggerEvent/${eventType}`;
    request.get(url)
      .end((err, res) => {
        if (err) {
          console.log(err);
          console.log(res);
          return;
        }
        console.log(`sent ${eventType}`);
      });
  }
};

