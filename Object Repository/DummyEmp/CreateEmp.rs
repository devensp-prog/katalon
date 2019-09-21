<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateEmp</name>
   <tag></tag>
   <elementGuidId>f2d07d91-5612-4d39-b2d8-1cae59533c8e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;${EmpName}\&quot;,\&quot;salary\&quot;:\&quot;${salary}\&quot;,\&quot;age\&quot;:\&quot;${age}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>95a34d61-cd34-49b3-8660-881b2e6c7222</id>
      <masked>false</masked>
      <name>EmpName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>31ab0971-534e-4801-ada2-6fde99647d6e</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ad19bda1-d567-409b-bdc9-eb6743084119</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
assertThat(response.responseBodyContent.contains(GlobalVariable.EmpName))



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
