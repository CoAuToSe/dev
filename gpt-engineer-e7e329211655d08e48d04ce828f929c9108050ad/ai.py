
import openai

import time

class AI:
    def __init__(self, **kwargs):
        self.kwargs = kwargs

    def start(self, system, user):
        messages = [
                {"role": "system", "content": system},
                {"role": "user", "content": user},
            ]

        return self.next(messages)

    def fsystem(self, msg):
        return {"role": "system", "content": msg}
    
    def fuser(self, msg):
        return {"role": "user", "content": msg}

    def next(self, messages: list[dict[str, str]], prompt=None):
        if prompt:
            messages = messages + [{"role": "user", "content": prompt}]
        time.sleep(1)  # Sleep for 1 second before making a new request
        response = openai.ChatCompletion.create(
            messages=messages,
            stream=True,
            **self.kwargs
        )

        chat = []
        for chunk in response:
            delta = chunk['choices'][0]['delta']
            msg = delta.get('content', '')
            print(msg, end="")
            time.sleep(1)  # Sleep for 1 second before making a new request
            chat.append(msg)
        return messages + [{"role": "assistant", "content": "".join(chat)}]