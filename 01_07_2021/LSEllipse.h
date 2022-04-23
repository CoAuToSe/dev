/************************************************************************* 
    Version: 2014-12-31 
         Function description: The least squares ellipse fitting is given to a series of points on the plane, using the singular value decomposition method 
                               The least square solution is obtained as the ellipse parameter. 
         Calling form: cvFitEllipse2f (arrayx, arrayy, box);     
         Parameter description: arrayx: arrayx[n], each value is a point on the x axis 
                               arrayx: arrayy[n], each value is a point on the y axis 
                               n: the number of points 
                               box: box[5], the five parameters of the ellipse, center.x, center.y, 2a, 2b, xtheta 
                               esp: Solution accuracy, usually 1e-6, this is used to solve equations 
***************************************************************************/  
  
  
  
  
// #include"stdafx.h"  
#include<cstdlib>  
#include<float.h>  
#include<vector>  
using namespace std;  
  
class LSEllipse  
{  
public:  
    LSEllipse(void); 
    ~LSEllipse(void);
    vector<double> getEllipseparGauss(vector<CPoint> vec_point);  
    void cvFitEllipse2f( int *arrayx, int *arrayy,int n,float *box );  
private:  
    int SVD(float *a,int m,int n,float b[],float x[],float esp);  
    int gmiv(float a[],int m,int n,float b[],float x[],float aa[],float eps,float u[],float v[],int ka);  
    int ginv(float a[],int m,int n,float aa[],float eps,float u[],float v[],int ka);  
    int muav(float a[],int m,int n,float u[],float v[],float eps,int ka);  
};  