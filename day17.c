#include <stdio.h>
#include <math.h>

int main()
{
    // int sx = 20;
    // int ex = 30;
    // int sy = -10;
    // int ey = -5;
    int sx = 288;
    int ex = 330;
    int sy = -96;
    int ey = -50;

    int lower_x = sqrt(sx);
    int upper_x = ex;
    int max_y = 0;
    // printf("")
    int count = 0;
    for (int xx = lower_x; xx <= upper_x; xx++)
    {
        int lower_y = sy;
        int upper_y = -sy;
        for (int yy = lower_y; yy <= upper_y; yy++)
        {

            int x = 0;
            int y = 0;
            int dx = xx;
            int dy = yy;
            while (x <= ex && y >= sy)
            {
                if (x >= sx && y <= ey)
                {
                    int tmp = yy * (yy - 1) / 2;
                    if (yy < 0)
                        tmp = 0;
                    if (tmp > max_y)
                    {
                        max_y = tmp;
                        printf("X: %d, Y: %d, xx: %d, yy:%d\n", x, y, xx, yy);
                    }
                    count++;
                    break;
                }
                x += dx;
                if (dx > 0)
                    dx--;
                if (dx < 0)
                    dx++;
                y += dy;
                dy--;
            }
        }
    }
    printf("Max Y: %d\nCount: %d\n", max_y, count);
    return 0;
}