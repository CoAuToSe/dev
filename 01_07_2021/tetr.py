import matplotlib.pyplot as plt
import scipy.optimize as so
import numpy as np

def my_fun(parameters, x_samples, y_samples):
    # Two focus coordinates and distance from the point to the two focus on the ellipse and as an optimized parameters
    x_focus_1,y_focus_1,x_focus_2,y_focus_2,sum_of_target_distance_between_edge_and_two_focus = parameters
    # Calculate the actual distance
    sum_of_actual_distance_between_edge_and_two_focus= \
        ((x_samples- x_focus_1) ** 2 + (y_samples-y_focus_1) ** 2) ** 0.5+\
          ((x_samples- x_focus_2) ** 2 + (y_samples-y_focus_2) ** 2) ** 0.5

    # print(np.average(sum_of_actual_distance_between_edge_and_two_focus))
    #      
    return np.sum(((sum_of_actual_distance_between_edge_and_two_focus
                    - sum_of_target_distance_between_edge_and_two_focus) ** 2)/(len(x_samples)-1))

def fit_ellipse(x_samples, y_samples):
    # Normalized
    vmax= max(np.max(x_samples), np.max(y_samples))
    x_samples= x_samples / vmax
    y_samples= y_samples / vmax
    # Optimization
    res_optimized = so.minimize(fun=my_fun, x0=np.array([-0.1,-0.05,0.1,0.1, 1.2]), args=(x_samples, y_samples))
    if res_optimized.success:
        print(res_optimized)
        x1_res, y1_res, x2_res, y2_res, l2_res = res_optimized.x
        # Generate an elliptic curve based on the optimized function
        # Calculate the elliptical index
        alpha_res= np.arctan((y2_res- y1_res)/(x2_res-x1_res))
        # Calculate the distance between the two focus
        l_ab= ((y2_res- y1_res)**2+ (x2_res-x1_res)**2)**0.5
        # Calculate long (short) axis length
        a_res= l2_res/2
        # Calculate short (length) axis length
        b_res=  ((l2_res/2)**2- (l_ab/2)**2)**0.5

        #     sequence
        theta_res = np.linspace(0.0, 6.28, 100)
        #      
        x_res = a_res * np.cos(theta_res) * np.cos(alpha_res) \
                - b_res * np.sin(theta_res) * np.sin(alpha_res)
        y_res = b_res * np.sin(theta_res) * np.cos(alpha_res) \
                + a_res * np.cos(theta_res) * np.sin(alpha_res)

        # plt.style.use("one")
        plt.axes([0.16, 0.15, 0.75, 0.75])
        plt.scatter(x_samples, y_samples, color="magenta", marker="+",
                    zorder=1, s=80, label="samples")
        plt.plot(x_res, y_res, color="deepskyblue", zorder=2,
                 label="fitted curve")
        plt.scatter(np.array([x1_res,x2_res]), np.array([y1_res,y2_res]),zorder=3,
                    color="r", label= "focus point")
        plt.xlabel("$x$")
        plt.ylabel("$y$")
        plt.legend()
        vmax = max(np.max(plt.xlim()), np.max(plt.ylim()))
        vmin = min(np.min(plt.xlim()), np.min(plt.ylim()))
        plt.ylim([1.1 * vmin - 0.1 * vmax, 1.1 * vmax - 0.1 * vmin])
        plt.xlim([1.25 * vmin - 0.25 * vmax, 1.25 * vmax - 0.25 * vmin])
        # plt.savefig("Figsave/a={:.3f};b={:.3f};theta={:.2f}deg.svg".format(a_res, b_res, alpha_res))
        plt.show()


if __name__== "__main__":
    theta_samples = np.linspace(0, 20, 1000)
    #     azimuth
    alpha_samples = -45.0 / 180.0 * np.pi
    # Long axis length
    a_samples = 6.0
    # Short axis length
    b_samples = 2.0
    # Sample X sequence and superimpose the random value of the positive distribution
    x_samples = a_samples * np.cos(theta_samples) * np.cos(alpha_samples) \
                - b_samples * np.sin(theta_samples) * np.sin(alpha_samples) \
                + np.random.randn(1000) * 0.05 * a_samples
    # Sample Y sequence and stack the random value of the positive distribution
    y_samples = b_samples * np.sin(theta_samples) * np.cos(alpha_samples) \
                + a_samples * np.cos(theta_samples) * np.sin(alpha_samples) \
                + np.random.randn(1000) * 0.05 * b_samples
    fit_ellipse(x_samples, y_samples)
