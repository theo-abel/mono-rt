using System;

namespace MonoRtTest {
    public class Player {
        public int health;
        public float speed;
        private bool active;
        public static int instanceCount;

        public void TakeDamage(int amount) {
            health -= amount;
        }

        public static void ResetCount() {
            instanceCount = 0;
        }
    }
}
