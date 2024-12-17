import test from 'ava'

import { plus100, pressKey, sendText } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

test('keypress', (t) => {
  pressKey('a', {
    meta: true,
  })
  sendText('discord')
  t.pass()
})
